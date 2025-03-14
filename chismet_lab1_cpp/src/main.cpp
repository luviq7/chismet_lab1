#include <iostream>
#include <math.h>
#include "functions.h"

using namespace std;

int main (){
    double a = 6;
    double b = 7;
    double eps;
    cout << "Введіть точність: ";
    cin >> eps;
    dix(0, a, b, eps);
    hord(0, a, b, eps);
}