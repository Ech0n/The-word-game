<script setup>
import { createConditionalExpression } from '@vue/compiler-core';
import Cell from './Cell.vue'
import Letter from './Letter.vue'

import { getCurrentInstance, reactive } from 'vue'
import axios from 'axios'

  var self = getCurrentInstance()
  // self.store = reactive({
  //   board:[]
  // })
  const board = reactive( Array.apply(null, Array(15*15)).map(function () {return 0;}))
  var lastreadletter = ''
  var word_start = undefined
  var next_letters =  Array.apply(null, Array(15)).map(function () {return 0;})
  var incolumn = undefined

  function drop(e){
    e.preventDefault();
    const data = JSON.parse(e.dataTransfer.getData("text"));
    let letter = data.value;
    var original_slot = data.inv
    let i  = Number(e.target.getAttribute('col'))
    var j = Number(e.target.getAttribute('row'))
    let id =(i*15)+j
    console.log(letter)
    board[id] = letter
    if(!word_start){
      word_start = [i,j,letter.charCodeAt(0)]
    }else if(incolumn == undefined){
      if(i==word_start[0]){
        incolumn = true;
        next_letters[j] = letter.charCodeAt(0);
      }else if(j==word_start[1]){
        incolumn = false
        next_letters[i] = letter.charCodeAt(0)
      }else{
        console.log("INWALID PLACEMENET")
      }
    }else{
      if(incolumn){
        next_letters[j] = letter.charCodeAt(0);
      }else{
        next_letters[i] = letter.charCodeAt(0)
      }
    }
    
  }
  function allowDrop(e){
    
    var data = e.dataTransfer.getData("text");
    let i  = e.target.getAttribute('col')
    var j = e.target.getAttribute('row')
    let id =(Number(i)*15)+Number(j)
    if(board[id] == 0 && e.target.className == "cell" && (word_start==undefined||(incolumn==undefined &&(word_start[0]==i||word_start[1]==j))|| (incolumn==true&&word_start[0]==i)||(incolumn==false&&word_start[1]==j) )){
      console.log("drop not prevented id:",e.target.className)
      e.preventDefault();
    }else{
      console.log("drop prevented")
    }
  }
  function reset(e){
    if(incolumn==undefined&&word_start){
      board[(word_start[0]*15)+word_start[1]] = 0
      word_start = undefined
    }
    else if(incolumn==true){
      let i = word_start[0];
      for(let j = 0;j<15;j++){
        console.log(j,next_letters[j],next_letters[j]!=0)
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
    console.log({
      letters:next_letters,
      column:incolumn,
      num:rowcolnum
    })
    console.log(next_letters,incolumn)
    axios.post('http://localhost:8080/word',{
      letters:next_letters,
      column:incolumn,
      num:rowcolnum
    }).then(response => {console.log(response)})
    next_letters = next_letters.map(function () {return 0;})
    incolumn= null
    word_start = null
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