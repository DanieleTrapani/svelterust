<script lang="ts">
  import { invalidateAll } from "$app/navigation";
  import type { PageData } from "./$types";

  export let data: PageData;
  let todos = data.todos;

  const deleteTodo = async (id: number) => {
    await fetch(`http://localhost:3000/delete/${id}`);
    invalidateAll();
  };

  const updateTodo = async (todo) => {
    await fetch(
      `http://localhost:3000/update?id=${todo.id}&description=${todo.description}&done=${todo.done}`
    );
  };
</script>

<div class="container mx-auto mt-16">
  <h1 class="h1 text-center">Todos</h1>

  <form action="http://localhost:3000/create" method="POST">
    <input
      class="input"
      type="text"
      name="description"
      placeholder="What needs to be done?"
      autocomplete="off"
    />
  </form>

  {#each todos as todo}
    <input
      type="checkbox"
      bind:checked={todo.done}
      class="checkbox"
      on:change={() => updateTodo(todo)}
    />
    <input
      type="text"
      class="input"
      bind:value={todo.description}
      placeholder="Description"
    />
    <p>{todo.done}</p>
    <button
      class="btn variant-filled-primary"
      on:click={() => updateTodo(todo)}
    >
      Update
    </button>
    <button
      class="btn variant-filled-primary"
      on:click={() => deleteTodo(todo.id)}
    >
      Delete
    </button>
  {/each}
</div>
