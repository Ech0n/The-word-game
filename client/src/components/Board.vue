<!-- TODO:
  Make parent component containing inventory and board
  Parent component contains main logic that manages the game state
  
-->
<script setup>
  import { createConditionalExpression } from '@vue/compiler-core';
  import Letter from './Letter.vue'
  import Inventory from './Inventory.vue'
  import { getCurrentInstance, reactive,ref } from 'vue'
  import axios from 'axios'

  var self = getCurrentInstance()
//TODO
// Remember letter slots and return them to their slots on reset

  const board = reactive( Array.apply(null, Array(15*15)).map(function () {return 0;}))
  var lastreadletter = ''
  var word_start = undefined
  var next_letters =  Array.apply(null, Array(15)).map(function () {return 0;})
  var incolumn = undefined
  const invRef = ref(null)
  let letters_taken_from_inv = []

  function drop(e){
    e.preventDefault();
    const data = JSON.parse(e.dataTransfer.getData("text"));
    let letter = data.value;
    var original_slot = data.inv
    let i  = Number(e.target.getAttribute('col'))
    var j = Number(e.target.getAttribute('row'))
    letters_taken_from_inv.push([original_slot,letter])
    let id =(i*15)+j
    board[id] = letter
    if(!word_start){
      word_start = [i,j,letter]
    }else if(incolumn == undefined){
      if(i==word_start[0]){
        incolumn = true;
        next_letters[j] = letter
      }else if(j==word_start[1]){
        incolumn = false
        next_letters[i] = letter
      }else{
        console.log("INWALID PLACEMENET")
      }
    }else{
      if(incolumn){
        next_letters[j] = letter
      }else{
        next_letters[i] = letter
      }
    }
    invRef.value.pop_letter(original_slot)
  }

  function allowDrop(e){
    
    var data = e.dataTransfer.getData("text");
    let i  = e.target.getAttribute('col')
    var j = e.target.getAttribute('row')
    let id =(Number(i)*15)+Number(j)
    if(board[id] == 0 && e.target.className == "cell" && (word_start==undefined||(incolumn==undefined &&(word_start[0]==i||word_start[1]==j))|| (incolumn==true&&word_start[0]==i)||(incolumn==false&&word_start[1]==j) )){
      e.preventDefault();
    }
  }

  function reset(){
    if(incolumn==undefined&&word_start){
      board[(word_start[0]*15)+word_start[1]] = 0
      word_start = undefined
    }
    else if(incolumn==true){
      let i = word_start[0];
      for(let j = 0;j<15;j++){
        if(next_letters[j]!=0){
          board[(i*15)+j] = 0
        }
      }
      next_letters = next_letters.map(function () {return 0;})
      incolumn= null
      board[(word_start[0]*15)+word_start[1]] = 0
      word_start = undefined
    }if(incolumn==false){
      let j = word_start[1];
      for(let i = 0;i<15;i++){
        if(next_letters[i]!=0){
          board[i*15+j] = 0
        }
      }
      next_letters =  next_letters.map(function () {return 0;})
      incolumn= null
      board[(word_start[0]*15)+word_start[1]] = 0
      word_start = undefined
    }
    letters_taken_from_inv.forEach(el=>{
      invRef.value.push_letter(el[0], el[1])
    })
    letters_taken_from_inv = []
  }
  function submit(){
    let rowcolnum = 0;
    if(!incolumn){
      next_letters[word_start[0]] = word_start[2]
      rowcolnum = word_start[1]
      incolumn = false
    }else{
      next_letters[word_start[1]] = word_start[2]
      rowcolnum = word_start[0]
    }
    // console.log({
    //   letters:next_letters,
    //   column:incolumn,
    //   num:rowcolnum
    // })
    // console.log(next_letters,incolumn)
    axios.post('http://localhost:8080/word',{
      letters:next_letters,
      column:incolumn,
      num:rowcolnum
    }).then(response => {
      if(response.data.valid == true)
      {
        invRef.value.fill(response.data.new_letters)
      }else{
        alert("Word does not exist")
        reset()
      }
      next_letters = next_letters.map(function () {return 0;})
      incolumn= null
      word_start = null
      letters_taken_from_inv = []
    })

  }
</script>
<!-- (i*15)+j -->

<template>
  <div class="board_container">
    <div v-for="i in 15" class="board_row"> 

      <div class="cell" v-for="j in 15" v-bind:col="i-1" v-bind:row="j-1"  @drop="drop" @dragover="allowDrop" >
        <Letter v-if="board[((i-1)*15)+(j-1)]" v-bind:value="board[((i-1)*15)+(j-1)]"  value="b"/> 
      </div>
      
    </div>
    
  </div>
  <button @click="reset"> reset</button>
  <button @click="submit"> submit</button>
  <Inventory ref="invRef"/>
</template>


<style scoped>
  .board_container{
    height: 750px;
    margin:50px 0px;
    display: grid;
    grid-template-columns: repeat(15, 1fr);
    grid-template-rows: repeat(15, 1fr);
    grid-column-gap: 0px;
    grid-row-gap: 0px;
  }
  .board_row{

  }
  .cell{
    width: 50px;
    height: 50px;
    border-color: white;
    border-style: solid;
    border-width: 2px;
    background-color: grey;
  }
</style>