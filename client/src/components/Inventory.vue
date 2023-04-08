<script setup>
import { getCurrentInstance, reactive } from 'vue'
import Letter from './Letter.vue'

var letters = reactive( Array.apply(null, Array(7)).map(function () {return 65;}))

function generate_lettr()
{
  const letter_count = 25
  const letter_start= 65;
  let i=0
  console.log("LITERA : ",i)
  letters[i]  = letter_start+Math.floor(Math.random()*letter_count)
  return String.fromCharCode(letters[i])
}

function add_letters(new_letters){
  let element = new_letters.shift()
  while( element != 0){

    element = new_letters.shift()
  }
}
function pop_letter(id){
  console.log("poppin ",id)
  letters[id] = 0
}
function push_letter(id,x){
  letters[id] = x
}
defineExpose({pop_letter})
</script>

<template>
    <div class="inventory">
        <table>
            <template v-for="i in 7">
                <td class="inv_cell" id="i_{{ i }}">
                    <Letter v-if="letters[i-1]" :value="String.fromCharCode(letters[i-1])" draggable="true"  :inv="i-1" > </Letter>
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