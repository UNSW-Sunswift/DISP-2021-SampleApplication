<script>
 import { onMount } from 'svelte';

 const zeroPad = (n) => {
   if (n < 10) return `0${n}`;
   return n;
 }

 let date = new Date();
 $: hour = zeroPad(date.getHours());
 $: min  = zeroPad(date.getMinutes());
 $: sec  = zeroPad(date.getSeconds());
 $: year = zeroPad(date.getFullYear());
 $: month  = zeroPad(date.getMonth());
 $: day  = zeroPad(date.getDay());

 onMount( () => {
   const interval = setInterval(() => {
     date = new Date();
   }, 1000);
 });
</script>

<style>
 #clock {
   font-size: 120px;
   text-align: center;
 }
 #date {
   font-size: 20px;
   text-align: center;
 }
</style>

<div>
    <div id="clock">
    {hour}:{min}:{sec}
    </div>
    <div id="date">
    {year}-{month}-{day}
    </div>
</div>
