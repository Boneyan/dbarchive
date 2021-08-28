'use strict';
function base64ToUint8Array(base64) {
    let raw = atob(base64);
    let uint8Array = new Uint8Array(raw.length);
    for (let i = 0; i < raw.length; i++) {
      uint8Array[i] = raw.charCodeAt(i);
    }
    return uint8Array;
  }

function ready() {
    //let pdfData = base64ToUint8Array(pdfData_s);
    /*let pdfViewerFrame = document.getElementById("pdfFrame");
    pdfViewerFrame.onload = function() {
        pdfViewerFrame.contentWindow.PDFViewerApplication.open(pdfData_s);
        console.log(12);
    }
    pdfViewerFrame.setAttribute("src","./pdfjs-dist/web/viewer.html?file=");*/
}

document.addEventListener("DOMContentLoaded", ready);

pdfjsLib.GlobalWorkerOptions.workerSrc =
'/scripts/pdf_js/build/pdf.worker.js';

// Opening PDF by passing its binary data as a string. It is still preferable
// to use Uint8Array, but string or array-like structure will work too.
var loadingTask = pdfjsLib.getDocument({ data: atob(pdfData), });
console.log(12);
loadingTask.promise.then(function(pdf) {
    console.log(120);
// Fetch the first page.
pdf.getPage(1).then(function(page) {
  var scale = 1.5;
  var viewport = page.getViewport({ scale: scale, });

  // Prepare canvas using PDF page dimensions.
  var canvas = document.getElementById('the-canvas');
  var context = canvas.getContext('2d');
  canvas.height = viewport.height;
  canvas.width = viewport.width;

  // Render PDF page into canvas context.
  var renderContext = {
    canvasContext: context,
    viewport: viewport,
  };
  page.render(renderContext);
});
});