<!-- no index -->

# Page Stats


<br>
<table id='page-load-stats'>
<thead><th>Event</th><th>Time (ms)</th></thead>
<tr>
<td>
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
">Load Timing Stats</button>
</td>
<td></td>
</tr>

</table>

<table>
  <thead><th>Key</th><th>Value</th></thead>
  <tr>
    <td>deployment status</td><td><md-block>
    <img src="https://github.com/ollielynas/md-website/actions/workflows/static.yml/badge.svg">
  </img>
</td>
<tr>
<td>last compiled</td><td>2024-05-23 21:40:27</td>
</tr>
<tr>
<td>number of pages</td><td>47</td>
</tr>
<tr>
<td>project size</td><td>168.7MiB</td>
</tr>
<!-- <tr>
<td>word count</td><td>0</td>
</tr> -->
</table>

<!-- LAST EDITED 1716457229 LAST EDITED-->