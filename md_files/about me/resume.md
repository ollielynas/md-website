<!-- no index -->

<style>
.md_file {
    overflow: visible;
}
#md_block {
    :not(:is(h1,h2,h3,h4,h5,h6,h7)):not(:last-child):not(.show):not(.show > *) {
        display: none;
    }
    .hide {
        display: none;
    }


    .show > button {
        display:none;
    }

    h1,h2,h3,h4,h5,h6 {
        font-weight: normal;
    }
    padding: 1em;
    background-color: white;
    filter: drop-shadow(0px 10px 3px black);

}
</style>

# Ollie Lynas
##### todo: make look good, this is more of a proof of concept
##### to generate a resume tick the items that you want and click generate


## Personal Values - [ ]
### Respect - [ ]
I respect ppl 

### Hard Work - [ ]
u gotta work hard

## Relevant Experience - [ ]

### Rust - [ ]
Rust is one of my favorite languages to use. I have been using it to develop a number fo personal project for several years. These include: 

 - [fortran 1 interpreter](/index.html#md_files/portfolio/desmos/raceing sim.md)
 - [city builder game](/index.html#md_files/portfolio/itch.io/mini city.md)
 - [web based re-randomizer](/index.html#md_files/portfolio/web/superfast re-randomiser.md)

In perticular I enjoyed the [web based re-randomizer](/index.html#md_files/portfolio/web/superfast re-randomiser.md) as I spent some time optimizing it to run as fast as possible. 

### Python - [ ]

Python was one of the first languages that I learnt. I have continued to use it to this day, however I mostly use it for 

### JavaScript - [ ]

I have used JS to create a number websites. Most notably is [capture the flag](/index.html#md_files/portfolio/team projects/capture the flag.md) which I made with a couple of friends.


## Qualifications - [ ]

## Prior Work Experience - [ ]

### TriStar - [ ]

I have worked at [TriStar](https://tristar.org.nz/) as a coach. 



<script>
function hashCode(str) {
    let hash = 0;
    for (let i = 0, len = str.length; i < len; i++) {
        let chr = str.charCodeAt(i);
        hash = (hash << 5) - hash + chr;
        hash |= 0; // Convert to 32bit integer
    }
    return hash;
}
</script>

<button onclick = "
md = document.getElementById('md_block');
hide = false;
for (i of md.children) {
    for (l of i.children) {
        if (l.checked == true) {
            hide = false
            l.className='hide'
        }
        if (l.checked == false) {
            hide = true
            l.className='hide'
        }
        if (l.checked == true) {
            hide = false
        }
    }
    if (!hide) {
        i.className='show'
    }else {
        i.className='hide'
    }
};
">generate</button>
<!-- LAST EDITED 1699415446 LAST EDITED-->