<script setup>
import Board from './components/Board.vue'
import Inventory from './components/Inventory.vue'

</script>

<template>
  <header>
    Letteraki
  </header>

  <main>
    <Board />
    <Inventory/>
  </main>
</template>

<style scoped>
header {
  line-height: 1.5;
}


@media (min-width: 1024px) {
  header {
    display: flex;
    flex-direction: column;
    place-items: center;
    padding-right: calc(var(--section-gap) / 2);
  }

  .logo {
    margin: 0 2rem 0 0;
  }

  header .wrapper {
    display: flex;
    place-items: flex-start;
    flex-wrap: wrap;
  }
}
</style>
