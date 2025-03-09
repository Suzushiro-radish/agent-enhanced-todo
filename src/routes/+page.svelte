<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { addTodo, getTodos, updateTodoStatus, deleteTodo, type Todo } from "../commands";
		// Svelte 5のRunes（$state）を使用してリアクティブな変数を宣言
	let title = $state("");
	let todos = $state<Todo[]>([]);
	let loading = $state(true);
	
	// TODOリストを更新する関数
	async function refreshTodos() {
		todos = await getTodos();
		loading = false;
	}
	
	// カスタムイベントのハンドラ
	function handleTodoUpdated() {
		refreshTodos();
	}
	
	// コンポーネントのマウント時にデータベースを初期化し、TODOリストを取得
	onMount(async () => {
		// カスタムイベントリスナーを追加
		refreshTodos();
		window.addEventListener('todo-updated', handleTodoUpdated);
	});
	
	// クリーンアップ
	onDestroy(() => {
		window.removeEventListener('todo-updated', handleTodoUpdated);
	});
	
	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
		if (title.trim() !== "") {
			await addTodo(title);
			title = ""; // 入力欄をクリア
		}
	}
	
	async function toggleTodoStatus(todo: Todo) {
		await updateTodoStatus(todo.id, !todo.done);
	}
	
	async function removeTodo(id: number) {
		await deleteTodo(id);
	}
</script>
<main class="container">
	<h1>TODOリスト</h1>
	
	<form onsubmit={handleSubmit} class="todo-form">
		<input 
			placeholder="タスクを入力..." 
			bind:value={title} 
			required 
			class="todo-input" 
		/>
		<button type="submit" class="add-button">追加</button>
	</form>
	
	{#if loading}
		<p>読み込み中...</p>
	{:else if todos.length === 0}
		<p class="no-todos">タスクがありません。新しいタスクを追加してください。</p>
	{:else}
		<ul id="todo-list">
			{#each todos as todo (todo.id)}
				<li class="todo-item">
					<input 
						type="checkbox" 
						checked={todo.done} 
						onchange={() => toggleTodoStatus(todo)}
					/>
					<span class={todo.done ? 'todo-done' : ''}>{todo.name}</span>
					<button class="delete-button" onclick={() => removeTodo(todo.id)}>削除</button>
				</li>
			{/each}
		</ul>
	{/if}
</main>
<style>
</style>
