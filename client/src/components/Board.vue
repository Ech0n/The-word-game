<script setup>
import Cell from './Cell.vue'
import Letter from './Letter.vue'
import { getCurrentInstance, reactive } from 'vue'

  var self = getCurrentInstance()
  // self.store = reactive({
  //   board:[]
  // })
  const board = reactive( Array.apply(null, Array(15*15)).map(function () {return 0;}))
  var lastreadletter = ''
  var word_start = undefined
  var next_letters = []
  var incolumn = undefined

  function drop(e){
    e.preventDefault();
    var data = e.dataTransfer.getData("text");
    let i  = Number(e.target.getAttribute('col'))
    var j = Number(e.target.getAttribute('row'))
    let id =(i*15)+j
    
    board[id] = data
    if(!word_start){
      word_start = [i,j]
    }else if(incolumn == undefined){
      if(i==word_start[0]){
        incolumn = true;
        next_letters.push(j)
      }else if(j==word_start[1]){
        incolumn = false
        next_letters.push( i)
      }else{
        console.log("INWALID PLACEMENET")
      }
    }else{
      if(incolumn){
        next_letters.push(j)
      }else{
        next_letters.push( i)
      }
    }
    console.log(next_letters)
    
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
      board[(word_start[0]*15)+word_start[1]] = null
      word_start = undefined
    }
    else if(incolumn==true){
      let i = word_start[0];
      for(let j = 0;j<next_letters.length;j++){
        board[(i*15)+next_letters[j]] = null
      }
      next_letters = []
      incolumn= null
      board[(word_start[0]*15)+word_start[1]] = null
      word_start = undefined
    }if(incolumn==false){
      let j = word_start[1];
      for(let i = 0;i<next_letters.length;i++){
        board[next_letters[i]*15+j] = null
      }
      next_letters = []
      incolumn= null
      board[(word_start[0]*15)+word_start[1]] = null
      word_start = undefined
    }
  }
  function submit(){
    next_letters = []
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