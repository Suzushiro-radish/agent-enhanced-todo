<script lang="ts">
	import { addTodo, getTodos } from "../commands";

	let title: string = "";
	let todos: { id: number, name: string, done: boolean }[] = [];

	async function handleSubmit() {
		await addTodo(title);
		todos = await getTodos();
	}
</script>

<main class="container">
	<h1>Welcome to Tauri + Svelte</h1>

	<form on:submit|preventDefault={handleSubmit}>
		<input placeholder="Enter a title..." bind:value={title} required class="todo-input" />
		<button type="submit" class="add-button">Add</button>
	</form>

	<ul id="todo-list">
		{#each todos as todo}
			<li>{todo.name}</li>
		{/each}
	</ul>
</main>

<style>
	.container {
		display: flex;
		flex-direction: column;
		align-items: center;
	}

	.todo-input {
		width: 200px;
		font-size: large;
	}

	.add-button {
		font-size: large;
		background-color: #4CAF50;
	}
</style>
