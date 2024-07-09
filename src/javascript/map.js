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

    let geojsonLayer;
    let geoJsonData;
    const parties = ["Grüne", "AfD", "BSW", "CDU", "Die Linke", "Die Partei", "FDP", , "SPD"];
    
    function updateLayer(party) {
      console.log('Update Layer function is called');
      if (!geoJsonData) {
        console.error('No GeoJSON data available');
        return;
    }
      if (geojsonLayer) {
          map.removeLayer(geojsonLayer);
          console.log('Layer is removed');
      }
      geojsonLayer = L.geoJSON(geoJsonData, {
        style: function(feature) {
          return style(feature, party);
        }
      }).addTo(map);
      console.log('Layer is loaded with party: ', party);

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
        let geoJsonLayer = L.geoJSON(data, {
            style: function(feature) {
                return style(feature, "Grüne");
            }
          });
          
          geoJsonLayer.addTo(map);
          console.log('Added GeoJSON layer to map');
          geoJsonData = data;
          updateLayer("Grüne");
          console.log('Initial layer added');
       } catch (error) {
      console.error('Error creating or adding GeoJSON layer:', error);
  }});

    console.log('Map finalized');
});