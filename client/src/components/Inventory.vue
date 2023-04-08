<script setup>
import { getCurrentInstance, onMounted, reactive , ref} from 'vue'
import Letter from './Letter.vue'
import axios from 'axios'

var letters = reactive( Array.apply(null, Array(7)).map(function () {return 65;}) )
const lettersListRef = ref([])

onMounted(() => {
  axios.get('http://localhost:8080/start_game').then(response => {
    for(let i =0;i<7;i++){
      push_letter(i,response.data.new_letters[i])
    }
  })
})

function generate_lettr()
{
  const letter_count = 25
  const letter_start= 65;
  let i=0
  letters[i]  = letter_start+Math.floor(Math.random()*letter_count)
  return String.fromCharCode(letters[i])
}

function pop_letter(id){
  letters[id] = 0
}
function push_letter(id,x){
  letters[id] = x
}
function fill(x){
  let j = 0;
  let i = 0
  while(i<7 && x[j]!= 0){
    if(letters[i] == 0){
      letters[i] = x[j]
      j++
    }
    i++
  }
}
defineExpose({pop_letter,push_letter,fill})
</script>

<template>
    <div class="inventory">
        <table>
            <template v-for="i in 7" >
                <td class="inv_cell" >
                    <Letter v-if="letters[i-1]" :value="letters[i-1]" draggable="true"  :inv="i-1" ref="lettersListRef" > </Letter>
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