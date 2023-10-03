
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
// 'redirectCount',
// 'redirectEnd',
// 'redirectStart',
'requestStart',
'responseEnd',
'responseStart',
// 'timing',
// 'navigation',
// 'performance',
// 'type',
// 'unloadEventEnd',
// 'unloadEventStart'        
];
var text='';
var p = document.getElementById('page-load-stats');
stats.sort((a,b)=>((window.performance.timing[a] - window.performance.timing.navigationStart)-(window.performance.timing[b] - window.performance.timing.navigationStart)))
for (i of stats) {
    text += '<tr><td>'+i+'</td><td>'+ (window.performance.timing[i] - window.performance.timing.navigationStart)+'</td><tr>'
};
console.log(text,p);
p.innerHTML = '<thead><th>Event</th><th>Time (ms)</th></thead>'+text;
">Get Load Stats</button>

<table id='page-load-stats'></table>