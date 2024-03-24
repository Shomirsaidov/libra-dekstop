<template>
	<div>
	  <!-- Your component template -->
	  <h1 class="text-black text-center text-lg font-black">{{ title }}</h1>
	  <p class="text-black">{{ text }}</p>
	</div>
  </template>
  
  <script>
  import DatabaseService from "../services/database";
  import readFile from '../services/fileread.js'
  
  export default {
	data: () => ({
		text: ''
	}),
	// Component options
	async mounted() {
	  try {
		// Example query
		const result = await DatabaseService.executeQuery(
			"SELECT * FROM barbers"
		);
		console.log("Query result:", result);
	  } catch (error) {
		console.error("Error:", error);
	  }

	  await readFile('file.txt').then(c => this.text = c);
	  console.log(this.text)

	},
  };
  </script>
  
  <style>
  /* Your component styles */
  </style>
  