<!-- no index -->
Firefox doesn't seem to like to update the cache so manual clearing may be necessary

<br>


<button
onclick = "
if ('serviceWorker' in navigator) {
navigator.serviceWorker
.getRegistrations()
.then(function (registrations) {
    for (let registration of registrations) {
        registration.unregister()
    }
})
}else{
    console.log('serviceWorker not in navigator')
}
window.location.reload(true) // works in firefox
window.location.reload() // just reloads in other browsers
">clear</button>

<!-- LAST EDITED 1700444242 LAST EDITED-->