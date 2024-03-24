<template>
  
    <div class="flex justify-center">
      <div>
        <video class="shadow-2xl border-2 rounded-lg border-b-8 border-blue-700" id="video" width="600" autoplay></video>
        <div>
          <h1 class="text-center my-2 text-black text-3xl" v-if="qrContent">QR content: 
           <a :href="qrContent">{{ qrContent }}</a></h1>

          <h1 v-else class="text-center my-2 text-blue-900 text-3xl">Please show your QR code to the camera</h1>
        </div>
      </div>
    </div>

</template>
  
  <script>


    import jsQR from 'jsqr'

    export default {
      name: 'scanner',
      data: () => ({
        qrContent: '',
      }),
      mounted() {
        const vueDataObject = this
        navigator.mediaDevices.getUserMedia({ video: true })
            .then(function (stream) {
            var video = document.getElementById('video');
            video.srcObject = stream;
            video.play();

            var canvas = document.createElement('canvas');
            var context = canvas.getContext('2d');

            video.addEventListener('loadedmetadata', function () {
                canvas.width = video.videoWidth;
                canvas.height = video.videoHeight;
            });

            video.addEventListener('play', function () {
                var self = this;
                (function loop() {
                if (!self.paused && !self.ended) {
                    context.drawImage(self, 0, 0, canvas.width, canvas.height);
                    var imageData = context.getImageData(0, 0, canvas.width, canvas.height);
                    var code = jsQR(imageData.data, imageData.width, imageData.height);

                    if (code) {
                    console.log('QR Code detected:', code.data);
                    vueDataObject.qrContent = code.data
                    }

                    vueDataObject.qrContent ? console.log('Go next, show user profile') : setTimeout(loop, 1000 / 30); // 30 fps
                }
                })();
            });
            })
            .catch(function (error) {
            console.error('Error accessing camera:', error);
            });
        }
    }
  </script>
  