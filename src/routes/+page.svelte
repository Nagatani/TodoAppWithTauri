<script>
  import { onMount } from 'svelte';
  import { todos, loadTodos, addTodo, updateTodoStatus, deleteTodo } from '../lib/stores/todoStore.js';

  let newTaskText = '';

  onMount(async () => {
    await loadTodos();
  });

  async function handleAddTodo() {
    if (newTaskText.trim() === '') return;
    await addTodo(newTaskText);
    newTaskText = ''; // Clear input after adding
  }

  async function handleToggleComplete(todo) {
    await updateTodoStatus(todo.id, !todo.completed);
  }

  async function handleDeleteTodo(todoId) {
    await deleteTodo(todoId);
  }
</script>

<main>
  <h1>Todo App</h1>

  <div class="add-todo">
    <input
      type="text"
      bind:value={newTaskText}
      placeholder="What needs to be done?"
      on:keypress={(e) => e.key === 'Enter' && handleAddTodo()}
    />
    <button on:click={handleAddTodo}>Add Todo</button>
  </div>

  {#if $todos.length === 0}
    <p>No todos yet! Add one above.</p>
  {:else}
    <ul>
      {#each $todos as todo (todo.id)}
        <li class:completed={todo.completed}>
          <span on:click={() => handleToggleComplete(todo)}>
            {todo.task}
          </span>
          <button class="delete" on:click={() => handleDeleteTodo(todo.id)}>Delete</button>
        </li>
      {/each}
    </ul>
  {/if}
</main>

<style>
  main {
    font-family: sans-serif;
    max-width: 600px;
    margin: 2em auto;
    padding: 1em;
    border: 1px solid #eee;
    border-radius: 5px;
  }

  h1 {
    color: #333;
    text-align: center;
  }

  .add-todo {
    display: flex;
    margin-bottom: 1em;
  }

  .add-todo input[type="text"] {
    flex-grow: 1;
    padding: 0.5em;
    border: 1px solid #ccc;
    border-radius: 3px;
  }

  .add-todo button {
    padding: 0.5em 1em;
    margin-left: 0.5em;
    background-color: #007bff;
    color: white;
    border: none;
    border-radius: 3px;
    cursor: pointer;
  }

  .add-todo button:hover {
    background-color: #0056b3;
  }

  ul {
    list-style: none;
    padding: 0;
  }

  li {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.5em 0;
    border-bottom: 1px solid #eee;
  }

  li:last-child {
    border-bottom: none;
  }

  li span {
    cursor: pointer;
    flex-grow: 1;
  }

  li.completed span {
    text-decoration: line-through;
    color: #aaa;
  }

  li button.delete {
    background-color: #dc3545;
    color: white;
    border: none;
    border-radius: 3px;
    padding: 0.3em 0.6em;
    cursor: pointer;
  }

  li button.delete:hover {
    background-color: #c82333;
  }
</style>
