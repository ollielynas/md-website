## Wordle

<!-- META the game Wordle made using only a mixture of google forms and google sheets META -->

This project is a recreation of [Wordle](https://www.nytimes.com/games/wordle/index.html) implemented using google sheets and google forms. the google sheet & form are shared between all players. every 7th submission will clear the wordle and generate a new word.

**to make a guess submit your word using the google sheet below**

you should not need to click "refresh google sheet" each time you submit however if nothing happens after a couple of seconds you should try clicking the button.

<button onclick="var iframe = document.getElementById('FrameID');iframe.src = iframe.src;">refresh google sheet</button>

<iframe id="FrameID" src="https://docs.google.com/spreadsheets/d/17LzKFD14EYlp2zvq8MLdOLiphvNO0tBZoIDnXPQyrPE/htmlembed/sheet?gid=367259484" height="240" width="90%"></iframe>


<br>

<iframe onload="var monitor = setInterval(function(){
    var elem = document.activeElement;
    if(elem && elem.tagName == 'IFRAME'){
        clearInterval(monitor);
        setTimeout(
    function() {
        var iframe = document.getElementById('FrameID');
        iframe.src = iframe.src;

    }, 1000);
}
}, 100);" src="https://docs.google.com/forms/d/e/1FAIpQLSeT9-owRH8ygfzdOOtc9s4rroqqnueQ72HEjxs0Rru-DGCiBA/viewform?embedded=true" frameborder="0" marginheight="0" marginwidth="0" height="600" width= "100%">Loading…</iframe>

[sheet link](https://docs.google.com/spreadsheets/d/17LzKFD14EYlp2zvq8MLdOLiphvNO0tBZoIDnXPQyrPE/edit#gid=764618375)
[form link](https://forms.gle/MRBL5jvXSpsMi4Ad8)