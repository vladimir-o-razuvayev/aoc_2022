import example from './example.txt';
import input from './input.txt';
import { highest_calories } from '../../../pkg';

const body = document.querySelector('body');

const select = document.createElement('select');
select.add(new Option('Example', "0"), undefined);
select.add(new Option('Input', "1"), undefined);
body.appendChild(select);

const p = document.createElement('p');
p.innerHTML = example;
body.appendChild(p);

const number_field = document.createElement('input');
number_field.type = 'number';
body.appendChild(number_field);

const p2 = document.createElement('p');
body.appendChild(p2);

number_field.onchange = function () {
    if (select.selectedIndex === 0) {
        p2.innerHTML = highest_calories(example, number_field.valueAsNumber);
    } else {
        p2.innerHTML = highest_calories(input, number_field.valueAsNumber);
    }
}