document.addEventListener('DOMContentLoaded', function() {
    // Initialize the map
    var map = L.map('map_lpz').setView([51.34, 12], 12);

    // Add a tile layer (replace with your preferred tile layer)
    L.tileLayer('https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png', {
        maxZom: 18,
        attribution: '© OpenStreetMap contributors'
    }).addTo(map);

    // TODO: leaflet expects WGS84 coordinates while we have UTM33 -> use geo crate 
    fetch('/assets/data/Leipzig.json')
    .then(response => response.json())
    .then(data => {
        L.geoJSON(data, {
            style: function(feature) {
                return {
                    color: "#ff7800",
                    weight: 2,
                    opacity: 0.65
                };
            }
        }).addTo(map);
    });

    // L.control.scale().addTo(map_lpz);

    console.log('Map initialized');
    console.log('Leaflet object:', L);
});