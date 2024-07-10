document.addEventListener('DOMContentLoaded', function() {
    var map = L.map('map_lpz').setView([51.34, 12.36], 12);

    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
        maxZom: 18,
        attribution: '© OpenStreetMap contributors'
    }).addTo(map);
    
    // adds scale to map
    L.control.scale().addTo(map);

    const colorScale = chroma.scale(['#FFEDA0', '#800026']).mode('lab');

    
    function getColor(d) {
      // return d > 30 ? '#800026' :
      // d > 25  ? '#BD0026' :
      // d > 20  ? '#E31A1C' :
      // d > 15  ? '#FC4E2A' :
      // d > 10   ? '#FD8D3C' :
      // d > 5   ? '#FEB24C' :
      // d > 2   ? '#FED976' :
      // '#FFEDA0';
      return colorScale(d / 40).hex();
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
    let currentParty = "Grüne"; 
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
      info.update(layer.feature.properties);
    }

    function resetHighlight(e) {
      geoJsonLayer.resetStyle(e.target);
      info.update();
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
  
    var info = L.control();

    info.onAdd = function (map) {
        this._div = L.DomUtil.create('div', 'info'); // create a div with a class "info"
        this.update();
        return this._div;
    };

    // TODO: update with property fields: feature.properties[party]
    info.update = function (props) {
        this._div.innerHTML = '<h4>Leipzig Stadtratswahl 2024</h4>' +  (props ?
            '<b>' + props.name_muni + '</b><br />' + 'Ergebnis: ' + currentParty + ' mit ' + props[currentParty] + '%' 
            : 'Hover over a state');
    };

    info.addTo(map);

    var legend = L.control({position: 'bottomright'});

    legend.onAdd = function (map) {
      var div = L.DomUtil.create('div', 'info legend'),
          grades = [0, 5, 10, 15, 20, 25, 30, 40];
          var height = 200;
          var width = 40;
   
      // Create the color scale bar
      var colorBar = '<div style="width:' + width + 'px; height:' + height + 'px; background: linear-gradient(to top, ' + 
      colorScale(0).hex() + ', ' + colorScale(1).hex() + 
      '); float:left; margin-right:10px;"></div>';

      // Create labels
      var labels = grades.map((grade, index) => {
      var y = height - (index * height / (grades.length - 1));
      return '<div style="position:absolute; left:' + (width + 12) + 'px; top:' + y + 'px;">' + grade + '%</div>';
      }).join('');

      div.innerHTML = '<div style="position:relative; height:' + height + 'px; padding-right: 40px;">' + colorBar + labels + '</div>';

      // div.innerHTML +=
      //         '<i style="background: linear-gradient(to right, ' + colorScale(0).hex() + ', ' +
      //         colorScale(1).hex() + ')"></i> ';

      // for (var i = 0; i < grades.length; i++) {
      //   div.innerHTML +=
      //       '<span style="float:left;">' + grades[i] + '</span>';
      // }

      return div;
    };

    legend.addTo(map);

    L.Control.PartySelect = L.Control.extend({
      onAdd: function(map) {
        const select = L.DomUtil.create('select', 'party-select');
        parties.forEach(party => {
          const option = L.DomUtil.create('option', '', select);
          option.value = party;
          option.textContent = party;
        });
        L.DomEvent.on(select, 'change', function() {
          currentParty = this.value;
          updateLayer(this.value);
        });
        return select;
      }
    });
    
    // adds selection of party on topright
    const partySelect = new L.Control.PartySelect({ position: 'topright' }).addTo(map);


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
          console.log('Data is defined: ', geoJsonData);
          updateLayer(currentParty);
          console.log('Initial layer added');
       } catch (error) {
      console.error('Error creating or adding GeoJSON layer:', error);
  }});

    console.log('Map finalized');
});
