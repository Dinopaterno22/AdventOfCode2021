#include <iostream>
#include <fstream>
#include <string>

uint32_t Sum(uint32_t a, uint32_t b, uint32_t c)
{
    return a + b + c;
}

void get_reading(std::ifstream &file, uint32_t &value)
{
    std::string s_current;
    file >> s_current;

    try
    {
        value = stoi(s_current);
    }
    catch (std::exception &ex)
    {
        std::cout << "Exception occurred!" << std::endl;
    }
}

void SingleMeasurement(std::ifstream &file)
{
    uint32_t i_current, i_previous, count = 0;
    std::string s_current;
    i_previous = i_current;
    get_reading(file, i_current);
    while (file)
    {
        i_previous = i_current;
        get_reading(file, i_current);
        if (i_current > i_previous)
            count++;
    }

    std::cout << "Single Measurement: " << count << std::endl;
}

void TripleMeasurement(std::ifstream &file)
{
    uint32_t i_a, i_b, i_c, i_current, i_previous, count = 0;
    std::string s_current;
    get_reading(file, i_a);
    get_reading(file, i_b);
    get_reading(file, i_c);

    i_current = Sum(i_a, i_b, i_c);
    i_previous = i_current;

    while (file)
    {
        i_a = i_b;
        i_b = i_c;
        get_reading(file, i_c);
        i_previous = i_current;
        i_current = Sum(i_a, i_b, i_c);
        if (i_current > i_previous)
            ++count;
    }
    std::cout << "Triple Measurement: " << count << std::endl;
}

int main()
{
    std::ifstream file;
    file.open("../input.txt");

    SingleMeasurement(file);
    file.close();              // clear fail and eof bits
    file.open("../input.txt"); // back to the start!
    TripleMeasurement(file);

    return 0;
}
