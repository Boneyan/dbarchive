'use strict';

let highlight = -1;
let alike_n = 0;
let helper = null;
let inp = null;

function clear() {
    helper.innerHTML = '';
    highlight = -1;
}

function help() {
    let val = document.getElementById('s-input').value;
    let ind = val.lastIndexOf(' ');
    if (ind + 1 < val.length) {
        helper.innerHTML = '';
        alike_n = 0;
        let subtag = val.slice(ind + 1).toLowerCase();
        tags.forEach(tag => {
            if (tag.toLowerCase().includes(subtag)) {
                let h = document.createElement('p');
                h.setAttribute('id', `h${alike_n}`);
                h.setAttribute('class', 'search-bar_helper-line');
                alike_n++;
                h.innerHTML = tag;
                helper.appendChild(h);
            }
        });
    } else {
        clear();
    }
}

function fill(txt) {
    let val= inp.value;
    let ind = val.lastIndexOf(' ');
    let new_val = val.slice(0, ind + 1) + txt;
    inp.value = new_val + ' ';
    inp.focus();
    helper.innerHTML = '';
}

function ready() {
    helper = document.getElementById('helper');
    inp = document.getElementById('s-input');

    document.getElementById('s-input').onfocus = help;
    document.getElementById('s-input').onblur = clear;
    document.getElementById('s-input').onkeyup = function(e) {
        let old_hgl = highlight;
        if (e.keyCode == 40) {
            if (highlight < alike_n - 1)
                highlight += 1;
        } else if (e.keyCode == 38) {
            if (highlight > -1)
                highlight -= 1;
        } else if (e.keyCode == 13) {
            let el = document.getElementById(`h${highlight}`);
            fill(el.innerHTML);
            //el.click();
        } else {
            clear();
            help();
        }
        if (highlight != old_hgl) {
            if (old_hgl >= 0) {
                let o_hgh = document.getElementById(`h${old_hgl}`);
                o_hgh.setAttribute('class', 'search-bar_helper-line');
            }
            if (highlight >= 0) {
                let hgh = document.getElementById(`h${highlight}`);
                hgh.setAttribute('class', 'search-bar_helper-line__selected');
            }
        }
    }
}

document.addEventListener("DOMContentLoaded", ready);