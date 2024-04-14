<script setup>
	import useWasm from "~/composables/wasm.client.js";

	const { greeting, input1, input2, result, using_rust_add } = useWasm();
</script>

<template>
	<main>
		<ClientOnly>
			<h1>{{ greeting }}</h1>
			<div class="loading"></div>

			<div class="set">
				<h1 v-if="!result">add 2 numbers 🧮</h1>
				<h1 v-else>{{ result }}</h1>
				<input type="number" v-model="input1" />
				<input type="number" v-model="input2" />
				<button @click="using_rust_add(input1, input2)">ADD</button>
			</div>
		</ClientOnly>
	</main>
</template>
