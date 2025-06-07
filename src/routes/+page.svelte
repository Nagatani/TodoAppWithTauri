<script lang="ts">
  import { onMount } from 'svelte';
  import { todos, loadTodos, addTodo, updateTodoStatus, deleteTodo, type Todo } from '../lib/stores/todoStore'; // Added 'type Todo'

  let newTaskText: string = ''; // Typed newTaskText
  let newDueDate: string = '';
  let sortByDueDateActive: boolean = false;

  onMount(async () => {
    await loadTodos();
  });

  async function handleAddTodo(): Promise<void> {
    if (newTaskText.trim() === '') return;
    // Pass null if newDueDate is empty, otherwise pass the string value
    await addTodo(newTaskText, newDueDate || null);
    newTaskText = '';
    newDueDate = ''; // Reset due date input
  }

  // Explicitly type the 'todo' parameter here for clarity
  async function handleToggleComplete(todo: Todo): Promise<void> { // Typed parameter and return
    await updateTodoStatus(todo.id, !todo.completed);
  }

  // Explicitly type 'todoId'
  async function handleDeleteTodo(todoId: number): Promise<void> { // Typed parameter and return
    await deleteTodo(todoId);
  }

  $: displayedTodos = sortByDueDateActive
    ? [...$todos].sort((a, b) => {
        const aHasDueDate = a.dueDate != null;
        const bHasDueDate = b.dueDate != null;

        if (aHasDueDate && !bHasDueDate) {
          return 1; // a (with due date) comes after b (without due date)
        }
        if (!aHasDueDate && bHasDueDate) {
          return -1; // a (without due date) comes before b (with due date)
        }
        if (aHasDueDate && bHasDueDate) {
          // Both have due dates, sort them chronologically
          return new Date(a.dueDate!).getTime() - new Date(b.dueDate!).getTime();
        }
        // Neither has a due date (or both are null), maintain original order (or sort by ID for stability)
        return a.id - b.id; // Optional: sort by ID for stable sort among items without due dates
      })
    : $todos;
</script>

<main>
  <h1>Todo App</h1>

  <div class="add-todo">
    <input
      type="text"
      bind:value={newTaskText}
      placeholder="What needs to be done?"
      on:keypress={(e: KeyboardEvent) => e.key === 'Enter' && handleAddTodo()}
      />
      <input
        type="datetime-local"
        bind:value={newDueDate}
    />
    <button on:click={handleAddTodo}>Add Todo</button>
  </div>

  <div class="controls" style="margin-bottom: 1em;">
    <button on:click={() => sortByDueDateActive = !sortByDueDateActive}>
      {sortByDueDateActive ? 'Clear Sort (Show All)' : 'Sort by Due Date'}
    </button>
  </div>

  {#if displayedTodos.length === 0}
    <p>No todos yet! Add one above.</p>
  {:else}
    <ul>
      {#each displayedTodos as todo (todo.id)}
        <li class:completed={todo.completed}>
          <!-- todo in '#each $todos as todo' will infer its type from $todos (Writable<Todo[]>) -->
          <span on:click={() => handleToggleComplete(todo)}>
            {todo.task}
            {#if todo.dueDate}
              <small style="margin-left: 10px; color: #666;">(Due: {new Date(todo.dueDate).toLocaleString()})</small>
            {/if}
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
