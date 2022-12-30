import example from './example.txt';
import input from './input.txt';
import { score } from '../../../pkg';

document.querySelector('.title').innerHTML = "Day 2";
const select = document.querySelector('#puzzle_input');
const code = document.querySelector('#display_input');
code.value = example;
const result = document.querySelector('#result');

select.onchange = function () {
    if (select.selectedIndex === 0) {
        code.value = example;
    } else {
        code.value = input;
    }
    result.innerHTML = score(code.value);
}