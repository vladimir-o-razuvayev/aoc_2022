import example from './example.txt';
import { highest_calories } from '../../../pkg';

const p = document.createElement('p');
const body = document.querySelector('body');
p.innerHTML = example;
body.appendChild(p);

const p2 = document.createElement('p');
p2.innerHTML = highest_calories(example, 3);
body.appendChild(p2);
