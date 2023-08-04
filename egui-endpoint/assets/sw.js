/* Inspired from egui-template */
var cacheName = 'egui-endpoint-pwa';
var filesToCache = [
  './',
  './index.html',
  './egui-endpoint.js',
  './egui-endpoint_bg.wasm',
];

self.addEventListener('install', function (e) {
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    })
  );
});

self.addEventListener('fetch', function (e) {
  e.respondWith(
    caches.match(e.request).then(function (response) {
      return response || fetch(e.request);
    })
  );
});
