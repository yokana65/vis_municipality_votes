document.addEventListener('DOMContentLoaded', function() {
    var map = L.map('map_lpz').setView([51.34, 12.36], 12);

    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
        maxZom: 18,
        attribution: '© OpenStreetMap contributors'
    }).addTo(map);
    
    // adds scale to map
    L.control.scale().addTo(map);

    const partyColorScales = {
      // continuous color from left to right
      "AfD": chroma.scale(['#D2B48C', '#4A2511']),
      "BSW": chroma.scale(['#FFE4B5', '#FF8C00']),
      "CDU": chroma.scale(['#D3D3D3', '#000000']),
      "Die Linke": chroma.scale(['#FFC0CB', '#FF1493']),
      "Die Partei": chroma.scale(['#FFCCCB', '#800020']),
      "FDP": chroma.scale(['#FFFACD', '#FFD700']),
      "Grüne": chroma.scale([ '#80ff00', '#009900']),
      "SPD": chroma.scale(['#FF0000', '#FFCCCB'])
    };
    
    function getColor(d, party) {
      const scale = partyColorScales[party] || chroma.scale(['#FFEDA0', '#800026']);
      const topRangeParty = topRange[party] || 40;
      return scale(d / topRangeParty).hex();
    }
    
    function style(feature, party) {
      return {
        fillColor: getColor(feature.properties[party], party),
        weight: 2,
        opacity: 1,
        color: 'white',
        dashArray: '3',
        fillOpacity: 0.7
      };
    }

    let geoJsonLayer;
    let geoJsonData;
    let legend;
    let currentParty = "Grüne"; 
    let topRange = {};
    const parties = ["Grüne", "AfD", "BSW", "CDU", "Die Linke", "Die Partei", "FDP", , "SPD"];
    
    function updateLayer(party) {
      console.log('Update Layer function is called');
      let currentParty = party;
      // const topRangeParty = topRange[party] || 40;
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
          return style(feature, party, topRange);
        },
        onEachFeature: onEachFeature
      }).addTo(map);
      console.log('Layer is loaded with party: ', party);

      updateLegend(party);
      console.log('Legend is updated');
    }

    function updateLegend(party) {
      if (legend) {
          map.removeControl(legend);
      }
      legend = L.control({position: 'bottomright'});
      legend.onAdd = function (map) {
          var div = L.DomUtil.create('div', 'info legend');
          var grades = [0, 5, 10, 15, 20, 25, 30, 40];
          // var grades = Array.from({length: 9}, (_, i) => (topRangeParty * i / 8).toFixed(1));
          var height = 200;
          var width = 30;
  
          var colorBar = '<div style="width:' + width + 'px; height:' + height + 'px; background: linear-gradient(to top, ' + 
                         partyColorScales[party](0).hex() + ', ' +
                         partyColorScales[party](1).hex() + 
                         '); float:left; margin-right:10px;"></div>';
  
          var labels = grades.map((grade, index) => {
              var y = height - (index * height / (grades.length - 1)) - 9;
              return '<div style="position:absolute; left:' + (width + 15) + 'px; top:' + y + 'px;">' + grade + '%</div>';
          }).join('');
  
          div.innerHTML = '<div style="position:relative; height:' + height + 'px; padding-right: 40px;">' + colorBar + labels + '</div>';
  
          return div;
      };
      legend.addTo(map);
    }

    function calculateMaxValues(data) {
      parties.forEach(party => {
          topRange[party] = Math.max(...data.features.map(feature => feature.properties[party] || 0));
      });
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
          updateLayer(this.value, topRange);
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
          let topRange = calculateMaxValues(data);
          console.log('Maximal Percentage for each party: ', topRange);
          updateLayer(currentParty);
          console.log('Initial layer added');
       } catch (error) {
      console.error('Error creating or adding GeoJSON layer:', error);
  }});

    console.log('Map finalized');
});
