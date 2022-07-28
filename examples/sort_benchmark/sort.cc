#include <iostream>
#include <vector>
#include <cstdlib>
#include <bits/stdc++.h>
#include <chrono>

using namespace std::chrono;

float rand_f()
{
    float r = (float)rand() / (float)RAND_MAX;
    return r;
}

int main(int argc, char **args)
{
    std::vector<float> *vec = new std::vector<float>(1000000);

    auto start = high_resolution_clock::now();
    for (int i = 0; i < 1000000; i++)
    {
        vec->push_back(rand_f());
    }
    std::sort(vec->begin(), vec->end());
    auto end = std::chrono::high_resolution_clock::now();
    auto duration = duration_cast<milliseconds>(end - start);
    std::cout << duration.count() << " ms"<<std::endl;
}