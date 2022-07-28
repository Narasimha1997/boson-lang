#include <iostream>
#include <vector>
#include <cstdlib>
#include <bits/stdc++.h>
#include <chrono>

using namespace std::chrono;
using namespace std;

#define m_time() chrono::high_resolution_clock::now()
#define m_diff(end, start) duration_cast<milliseconds>(end - start).count()
#define m_sort(vec) sort(vec->begin(), vec->end())

float rand_f()
{
    float r = (float)rand() / (float)RAND_MAX;
    return r;
}

int main(int argc, char **args)
{
    auto vec = new vector<float>(1000000);
    auto start = m_time();
    for (int i = 0; i < 1000000; i++)
    {
        vec->push_back(rand_f());
    }
    m_sort(vec);
    auto end = m_time();
    auto time = m_diff(end, start);
    cout << time << " ms"<<std::endl;
}