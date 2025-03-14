#include "functions.h"
#include <math.h>
#include <iostream>
#include <iomanip>
using namespace std;
double dix(int n, double a, double b, double eps){
    double x = (a + b) / 2;
    if(n == 0){
        cout << "Метод дихотомії" << endl;
        cout << left << setw(5) << "N" << setw(10) << "a" << setw(10) << "b" << setw(12) << "|a-b|" << setw(10) << "eps" << setw(12) << "x" << endl;
        cout << string(55, '-') << endl;
        cout << left << setw(5) << n << setw(10) << a << setw(10) << b << setw(12) << abs(a - b) << setw(10) << eps << setw(12) << x << endl;
    }
    if(abs(a-b)<eps || func(x) == 0){
        return x;
    }
    else{
        if(func(a)*func(x)>0){
            a = x;
        }
        if(func(b)*func(x)>0){
            b = x;
        }
        if(n>0){
            cout << left << setw(5) << n << setw(10) << a << setw(10) << b << setw(12) << abs(a - b) << setw(10) << eps << setw(12) << x << endl;
        }
        return dix(n+1, a, b, eps);
    }
}
double hord(int n, double a, double b, double eps){
    double x = a - (func(a)*(b-a))/(func(b)-func(a));
    if(n == 0){
        cout << "Метод хорд" << endl;
        cout << left << setw(5) << "N" << setw(10) << "a" << setw(10) << "b" << setw(12) << "|a-x|" << setw(10) << "eps" << setw(12) << "x" << endl;
        cout << string(55, '-') << endl;
        cout << left << setw(5) << n << setw(10) << a << setw(10) << b << setw(12) << abs(a - b) << setw(10) << eps << setw(12) << x << endl;
    }
    if(abs(a-x)<eps || func(x) == 0){
        return x;
    }
    else{
        if(n>0){
            cout << left << setw(5) << n << setw(10) << a << setw(10) << b << setw(12) << abs(a - x) << setw(10) << eps << setw(12) << x << endl;
        }
        return hord(n+1, x, b, eps);
    }
    

}



double func(double x){
    return pow(x,3) - 6 * pow(x,2) - 7;
}