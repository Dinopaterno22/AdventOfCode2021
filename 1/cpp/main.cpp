#include <iostream>
#include <fstream>
#include <string>

int main()
{
    std::ifstream file;
    file.open("../input.txt");

    uint32_t i_current, i_previous, count = 0;
    std::string s_current;
    i_previous = i_current;
    file >> s_current;
    i_current = stoi(s_current);
    while (file)
    {
        i_previous = i_current;
        file >> s_current;
        i_current = stoi(s_current);
        if (i_current > i_previous)
        {
            count++;
        }
    }

    std::cout << count;

    return 0;
}