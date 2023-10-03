
![example workflow](https://github.com/ollielynas/md-website/actions/workflows/static.yml/badge.svg)



<button onclick="
stats = [
'connectEnd',
'connectStart',
'domComplete',
'domContentLoadedEventEnd',
'domContentLoadedEventStart',
'domInteractive',
'domLoading',
'domainLookupEnd',
'domainLookupStart',
'fetchStart',
'loadEventEnd',
'loadEventStart',
'navigationStart',
'redirectCount',
'redirectEnd',
'redirectStart',
'requestStart',
'responseEnd',
'responseStart',
'timing',
'navigation',
'performance',
'type',
'unloadEventEnd',
'unloadEventStart'        
];
var text='';
var p = document.getElementById('page-load-stats');
for (i in stats) {
    text += '\n'+i
};
p.innerText = text;
"></button>

<p id='page-load-stats'>no stats calculated...</p>