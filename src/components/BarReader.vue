<template>
  
  <div class="flex justify-center">
      <div>
        <video class="shadow-2xl border-2 rounded-lg border-b-8 border-blue-700" id="video" width="600" autoplay></video>
        <div>
          <h1 class="text-center my-2 text-black text-3xl" v-if="barcodeContent">Barcode content: <span class="font-bold text-blue-900">{{barcodeContent[0]}} &nbsp{{barcodeContent.slice(1,7)}} &nbsp{{barcodeContent.slice(7,barcodeContent.length)}}</span></h1>
          <h1 v-else class="text-center my-2 text-blue-900 text-3xl">Please show a barcode to the camera</h1>
        </div>
      </div>
  </div>

  </template>
  
  <script>
  import Quagga from 'quagga';
  
  export default {
    name: 'BarcodeScanner',
    data() {
      return {
        barcodeContent: '',
      };
    },
    mounted() {
      this.initCamera();
    },
    methods: {
      async initCamera() {
        try {
          const stream = await navigator.mediaDevices.getUserMedia({ video: true });
          const video = document.getElementById('video');
          video.srcObject = stream;
  
          Quagga.init({
            inputStream: {
              name: 'Live',
              type: 'LiveStream',
              target: video,
            },
            decoder: {
              readers: ['ean_reader', 'code_128_reader'],
            },
          }, (err) => {
            if (err) {
              console.error('Error initializing Quagga:', err);
              return;
            }
            Quagga.start();
  
            Quagga.onDetected((result) => {
              this.barcodeContent = result.codeResult.code;
            });
          });
        } catch (error) {
          console.error('Error accessing camera:', error);
        }
      },
    },
    beforeDestroy() {
      Quagga.stop();
    },
  };
  </script>
  
  <style scoped>
  /* Add any necessary styles here */
  </style>
  