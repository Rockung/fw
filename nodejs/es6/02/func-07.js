"use strict";

function getContext() {
    console.log(this); // Global or Window
}
getContext()