<script>
 import Button from '@smui/button';

// This is the only part of the code that's specific to
// webview. Everything else works like a normal Svelte
// application.
var rpc = {
  // This is the function that directly sends stuff to the backend :)
   invoke : function(arg) { window.external.invoke(JSON.stringify(arg)); },

  // `rpc.init()` sends the 'init' cmd to the backend.
  init : function() { rpc.invoke({cmd : 'init'}); },

   // `rpc.log()` sends the 'log' cmd to the backend, passing
   // all arguments as a single string.
  log : function() {
    var s = '';
    for (var i = 0; i < arguments.length; i++) {
      if (i != 0) {
        s = s + ' ';
      }
      s = s + JSON.stringify(arguments[i]);
    }
    rpc.invoke({cmd : 'log', text : s});
  },
 // To add an extra backend call, just add a function that works
 // like the ones above. Then, in your javascript, call `rpc.that_function`
 };


// Everthing webview related pretty much ends here.
</script>


<style>
    section {
    padding: 5px;
    margin: 5px 0px; /* top/bottom l/r */
    }

    .flex-col {
    display: flex;
    flex-direction: column;
    align-items: stretch;
    }
    .flex-row {
    display: flex;
    flex-direction: row;
    align-items: stretch;
    }
    main {
    flex: 0 0 100%;
    height: 100%;
    }

    /* Colors, for layout demoing  */
    .test1 {
    background-color: #dddd99;
    }
    .test2 {
    background-color: #99dddd;
    }
    .test3 {
    background-color: #dd99dd;
        }

    /* Main Section, either displays 80/20 if fullscreen or control only */
    @media (min-height: 1000px)    {
    #content {
        flex-grow: 8;
    }
    #control {
        flex-grow: 2;
    }
    }
    @media not (min-height: 1000px)    {
    #content {
        display: none;
    }
    #control {
        flex-grow: 1;
    }
    }

    #control {
    display: flex;
    }

    #widgets {
    flex-grow: 1;
    margin: 2px;
    }
    #appbar  {
    flex-grow: 2;
    margin: 2px;
    }

    .widget {
    background-color: #dddddd;
    margin: 2px;
    width: 100%;
    flex-grow: 1;
    }

    .icon {
    background-color: #bbbbbb;
    margin: 2px;
    width: 20%;
    height: 20%;
    }

</style>

<main class="flex-col">
    <section id="content" class="test1 main-section">
    <p>Test</p>
    </section>
    <section id="control" class="test2 main-section flex-row">
    <div id="widgets" class="flex-col">
        <div class="widget">
        <p>Current Time</p>
        </div>
        <div class="widget">
        <p>Playing Now</p>
        </div>
    </div>
    <div id="appbar" class="flex-row">
        <div class="icon"><Button on:click={open_spotify}>Spotify</Button></div>
    </div>
    </section>
</main>
