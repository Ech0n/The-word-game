<script setup>
import { setBlockTracking } from 'vue';
import { getCurrentInstance, reactive } from 'vue'
var self = getCurrentInstance()

// import {  onMounted } from 'vue'
const props = defineProps({
  value: {
    type: String,
    required: true
  },
  random:{
    type: Boolean,
  },
  // TODO: Delete this (inv:) and implement referencing slots of inventory through list ref
  inv:{
    type:Number
  }
})
const position = {

}
function move(e){
  // e.dataTransfer.setData('text/plain', e.target.id);
  e.dataTransfer.setData("text", JSON.stringify(props));
  e.currentTarget.style.backgroundColor = 'black';
  self.attrs.draggable = "false"
}
function dragy(e){
}


</script>

<template>
  <div class="letter"  @dragstart="move" @drag="dragy"   >
    {{ value }}
  </div>
</template>


<style scoped>
  .letter{
    text-align: center;
    line-height: 46px;
    font-size:25pt;
    width: 48px;
    height: 48px;
    border-color: black;
    border-style: solid;
    border-width: 1px;
    border-radius: 5px;
    background-color: grey;
  }
</style>
