import example from './example.txt';
import input from './input.txt';
import { highest_calories } from '../../../pkg';

const body = document.querySelector('#content');

const select = document.createElement('select');
select.add(new Option('Example', "0"), undefined);
select.add(new Option('Input', "1"), undefined);
body.appendChild(select);

const div = document.createElement('div')
div.style = 'width:80px; height:280px; border:solid; overflow:scroll;';
body.appendChild(div);

const code = document.createElement('code');
code.style = 'white-space: pre-wrap'
code.innerHTML = example;
div.appendChild(code);

const number_field = document.createElement('input');
number_field.type = 'number';
number_field.setAttribute('min', '0');
number_field.defaultValue = '0';
body.appendChild(number_field);

const p2 = document.createElement('p');
p2.innerHTML = '0';
body.appendChild(p2);

number_field.onchange = function () {
    if (select.selectedIndex === 0) {
        p2.innerHTML = highest_calories(example, number_field.valueAsNumber);
    } else {
        p2.innerHTML = highest_calories(input, number_field.valueAsNumber);
    }
}

select.onchange = function () {
    if (select.selectedIndex === 0) {
        p2.innerHTML = highest_calories(example, number_field.valueAsNumber);
        code.innerHTML = example;
    } else {
        p2.innerHTML = highest_calories(input, number_field.valueAsNumber);
        code.innerHTML = input;
    }
}