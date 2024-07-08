document.addEventListener('DOMContentLoaded', function() {
    // Initialize the map
    var map = L.map('map_lpz').setView([51.34, 12], 12);

    // Add a tile layer (replace with your preferred tile layer)
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
        maxZom: 18,
        attribution: '© OpenStreetMap contributors'
    }).addTo(map);

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
      console.log('Parsed JSON data:', data);
      
      if (!data || typeof data !== 'object') {
        throw new Error('Invalid GeoJSON data');
      }
  
      try {
        const geoJsonLayer = L.geoJSON(data, {
          style: function(feature) {
            console.log('Styling feature:', feature);
            return {
              color: "#ff7800",
              weight: 2,
              opacity: 0.65
            };
          }
        });
  
        console.log('Created GeoJSON layer:', geoJsonLayer);
  
        geoJsonLayer.addTo(map);
        console.log('Added GeoJSON layer to map');
      } catch (error) {
        console.error('Error creating or adding GeoJSON layer:', error);
      }
    })
    .catch(error => {
      console.error('Error loading or processing GeoJSON:', error);
    });
  
  // Add this line at the end
  console.log('Fetch request initiated');

    L.control.scale().addTo(map);

    console.log('Map initialized');
    console.log('Leaflet object:', L);
});