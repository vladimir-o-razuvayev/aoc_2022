import example from './example.txt';
import input from './input.txt';
import { highest_calories } from '../../../pkg';

const title = document.querySelector('.title');
title.innerHTML = "Day 1";
const select = document.querySelector('#puzzle_input');
const code = document.querySelector('#display_input');
code.value = example;
const number_field = document.querySelector('#puzzle_value');
const result = document.querySelector('#result');

number_field.onchange = function () {
    result.innerHTML = highest_calories(code.value, number_field.valueAsNumber);
}

select.onchange = function () {
    if (select.selectedIndex === 0) {
        code.value = example;
    } else {
        code.value = input;
    }
    result.innerHTML = highest_calories(code.value, number_field.valueAsNumber);
}