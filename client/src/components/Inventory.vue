<script setup>
import Letter from './Letter.vue'

function generate_lettr()
{
  const letter_count = 25
  const letter_start= 65;
  return String.fromCharCode(letter_start+Math.floor(Math.random()*letter_count))
}
</script>

<template>
    <div class="inventory">
        <table>
            <template v-for="i in 7">
                <td class="inv_cell" id="i_{{ i }}">
                    <Letter :value="generate_lettr()" draggable="true" > </Letter>
                </td>
            </template>
        </table>
    </div>
</template>

<style scoped>
  .inventory{
    width: 370px;
    height: 60px;
    border-color: white;
    border-style: solid;
    border-width: 2px;
    background-color: grey;
  }
  .inv_cell{
    width:50px;
    margin:0px;
    text-align: center;
    line-height: 40px;
  }
</style>