<script lang="ts">
  import { onMount } from 'svelte';
  import { todos, loadTodos, addTodo, updateTodoStatus, deleteTodo, type Todo } from '../lib/stores/todoStore'; // Added 'type Todo'

  let newTaskText: string = ''; // Typed newTaskText
  let newDueDate: string = '';
  let sortColumn: keyof Todo | null = null; // Column to sort by
  let sortDirection: 'asc' | 'desc' = 'asc'; // Sort direction

  onMount(async () => {
    await loadTodos();
  });

  async function handleAddTodo(): Promise<void> {
    if (newTaskText.trim() === '') return;

    // Log the values just before using them
    console.log('[handleAddTodo] Attempting to add todo. Task:', newTaskText, 'Raw newDueDate:', newDueDate);

    const dueDateToSend = newDueDate || null;
    console.log('[handleAddTodo] Due date to send to store:', dueDateToSend);

    await addTodo(newTaskText, dueDateToSend);
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

  function handleSort(column: keyof Todo) {
    if (sortColumn === column) {
      sortDirection = sortDirection === 'asc' ? 'desc' : 'asc';
    } else {
      sortColumn = column;
      sortDirection = 'asc';
    }
  }

  $: displayedTodos = [...$todos].sort((a, b) => {
    if (!sortColumn) return (a.id < b.id) ? -1 : 1; // Default sort by ID if no column selected for stability

    const valA = a[sortColumn];
    const valB = b[sortColumn];

    let comparison = 0;

    switch (sortColumn) {
      case 'task':
        comparison = (valA as string).localeCompare(valB as string);
        break;
      case 'dueDate':
        const dateA = valA ? new Date(valA as string).getTime() : null;
        const dateB = valB ? new Date(valB as string).getTime() : null;
        if (dateA === null && dateB === null) comparison = 0;
        else if (dateA === null) comparison = 1; // nulls last for asc
        else if (dateB === null) comparison = -1; // nulls last for asc
        else comparison = dateA - dateB;
        break;
      case 'completed':
        if ((valA as boolean) === (valB as boolean)) comparison = 0;
        else comparison = (valA as boolean) ? 1 : -1; // True (completed) after False (pending)
        break;
      default:
        // For 'id' or other non-specified sortable columns, sort by id
        comparison = (a.id < b.id) ? -1 : 1; // Fallback to id sort
        break;
    }
    return sortDirection === 'asc' ? comparison : -comparison;
  });
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

  {#if displayedTodos.length === 0}
    <p>No todos yet! Add one above.</p>
  {:else}
    <table>
      <thead>
        <tr>
          <th on:click={() => handleSort('task')} style="cursor: pointer;">Task</th>
          <th on:click={() => handleSort('dueDate')} style="cursor: pointer;">Due Date</th>
          <th on:click={() => handleSort('completed')} style="cursor: pointer;">Status</th>
          <th>Actions</th> <!-- Not sortable -->
        </tr>
      </thead>
      <tbody>
        {#each displayedTodos as todo (todo.id)}
          <tr class:completed={todo.completed}>
            <td on:click={() => handleToggleComplete(todo)} style="cursor: pointer;">
              {todo.task}
            </td>
            <td>
              {#if todo.dueDate}
                {new Date(todo.dueDate).toLocaleString()}
              {:else}
                N/A
              {/if}
            </td>
            <td>
              {todo.completed ? 'Completed' : 'Pending'}
            </td>
            <td>
              <button class="delete" on:click={() => handleDeleteTodo(todo.id)}>Delete</button>
            </td>
          </tr>
        {/each}
      </tbody>
    </table>
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

  /* Table styles */
  table {
    width: 100%;
    border-collapse: collapse;
    margin-top: 1em;
  }

  th, td {
    border: 1px solid #ddd;
    padding: 8px 12px;
    text-align: left;
    vertical-align: middle;
  }

  th { /* Headers */
    background-color: #f2f2f2;
    font-weight: bold;
  }

  /* Clickable headers (assuming inline style="cursor:pointer" exists from previous step) */
  th[style*="cursor: pointer"]:hover {
      background-color: #e8e8e8;
  }

  /* Task cell (first child) in a completed row */
  tr.completed td:first-child {
    text-decoration: line-through;
    color: #aaa;
  }

  /* Clickable task cell for toggling (assuming inline style="cursor:pointer" exists on it) */
  td[style*="cursor: pointer"]:hover {
    background-color: #f5f5f5;
  }
</style>
