document.addEventListener('DOMContentLoaded', function() {
    var map = L.map('map_lpz').setView([51.34, 12.36], 12);

    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
        maxZom: 18,
        attribution: '© OpenStreetMap contributors'
    }).addTo(map);
    
    // adds scale to map
    L.control.scale().addTo(map);
    
    function getColor(d) {
      return d > 5000 ? '#800026' :
      d > 3000  ? '#BD0026' :
      d > 2500  ? '#E31A1C' :
      d > 2000  ? '#FC4E2A' :
      d > 1500   ? '#FD8D3C' :
      d > 1000   ? '#FEB24C' :
      d > 500   ? '#FED976' :
      '#FFEDA0';
    }
    
    function style(feature, party) {
      return {
        fillColor: getColor(feature.properties[party]),
        weight: 2,
        opacity: 1,
        color: 'white',
        dashArray: '3',
        fillOpacity: 0.7
      };
    }

    let geoJsonLayer;
    let geoJsonData;
    const parties = ["Grüne", "AfD", "BSW", "CDU", "Die Linke", "Die Partei", "FDP", , "SPD"];
    
    function updateLayer(party) {
      console.log('Update Layer function is called');
      if (!geoJsonData) {
        console.error('No GeoJSON data available');
        return;
    }
      if (geoJsonLayer) {
          map.removeLayer(geoJsonLayer);
          console.log('Layer is removed');
      }

      // function onEachFeature(feature, layer) {
      //   layer.on({
      //       mouseover: highlightFeature,
      //       mouseout: resetHighlight,
      //       click: zoomToFeature
      //   });
      // }

      geoJsonLayer = L.geoJSON(geoJsonData, {
        style: function(feature) {
          return style(feature, party);
        },
        onEachFeature: onEachFeature
      }).addTo(map);
      console.log('Layer is loaded with party: ', party);

    }

    function highlightFeature(e) {
      var layer = e.target;
  
      layer.setStyle({
          weight: 5,
          color: '#666',
          dashArray: '',
          fillOpacity: 0.7
      });
  
      layer.bringToFront();
    }

    function resetHighlight(e) {
      geojson.resetStyle(e.target);
    }

    function zoomToFeature(e) {
      map.fitBounds(e.target.getBounds());
    }

    function onEachFeature(feature, layer) {
      layer.on({
          mouseover: highlightFeature,
          mouseout: resetHighlight,
          click: zoomToFeature
      });
    }
  

    L.Control.PartySelect = L.Control.extend({
      onAdd: function(map) {
        const select = L.DomUtil.create('select', 'party-select');
        parties.forEach(party => {
          const option = L.DomUtil.create('option', '', select);
          option.value = party;
          option.textContent = party;
        });
        L.DomEvent.on(select, 'change', function() {
          updateLayer(this.value);
        });
        return select;
      }
    });
    
    // adds selection of party on topright
    const partySelect = new L.Control.PartySelect({ position: 'topright' }).addTo(map);

    // var info = L.control();

    // info.onAdd = function (map) {
    //     this._div = L.DomUtil.create('div', 'info'); // create a div with a class "info"
    //     this.update();
    //     return this._div;
    // };

    // // method that we will use to update the control based on feature properties passed
    // info.update = function (props) {
    //     this._div.innerHTML = '<h4>US Population Density</h4>' +  (props ?
    //         '<b>' + props.name + '</b><br />' + props.density + ' people / mi<sup>2</sup>'
    //         : 'Hover over a state');
    // };

    // info.addTo(map);

    console.log('Starting fetch request');
    fetch('/assets/data/Leipzig.json')
    .then(response => {
      console.log('Received response:', response);
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      return response.json();
    })
    .then(data => {
      
      if (!data || typeof data !== 'object') {
        throw new Error('Invalid GeoJSON data');
      }

      try {
          geoJsonData = data;
          updateLayer("Grüne");
          console.log('Initial layer added');
       } catch (error) {
      console.error('Error creating or adding GeoJSON layer:', error);
  }});

    console.log('Map finalized');
});