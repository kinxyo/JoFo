<script setup lang="ts">
	import init, { greet, formatter } from "~/WASM/wasmfile.js";

	const greetings: Ref<string> = ref("");
	const txt: Ref<string> = ref("");

	onBeforeMount(() => {
		init().then(() => {
			greetings.value = greet();
      startLiveEditing();
		});
	});

	async function startLiveEditing() {
		watchEffect(() => {
      let data: string = formatter(txt.value);
      console.log(data);
		});
	}

  const formattedText = computed(() => {
  let data: string = formatter(txt.value);
  return data.map(token => {
    switch (token.symbol_type) {
      case 'sidenote':
        return `<span class="sidenote">${token.text}</span>`;
      case 'emphasize':
        return `<span class="emphasize">${token.text}</span>`;
      default:
        return token.text;
    }
  }).join('');
});
</script>

<template>
	<main class="app">
		<ClientOnly>
			<header>{{ greetings }}</header>
			<textarea v-model="txt"> </textarea>
      <div v-html="formattedText"></div>
		</ClientOnly>
	</main>
</template>

<style scoped>
	.app {
		height: 100vh;
		width: 100vw;
	}

	header {
		display: flex;
		justify-content: space-between;
		padding-inline: 2rem;
		padding-block: 1rem;

		background-color: var(--ui);
		border-bottom-right-radius: 1rem;
		border-bottom-left-radius: 1rem;
	}

	textarea {
		margin-top: 0.2rem;
		background-color: var(--ui);
		color: var(--silver);

		border: none;
		border-top-right-radius: 1rem;
		border-top-left-radius: 1rem;

		height: 100vh;
		width: 100vw;

		font-family: system-ui;
		font-size: 12.5px;

		padding: 1rem;
	}

	textarea:focus {
		outline: none;
	}
</style>
