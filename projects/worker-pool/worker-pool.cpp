#include <iostream>
#include <thread>
#include <vector>

using namespace std;

vector<char> realBuffer;

bool exit_prg = false;
bool input_dirty = false;

void handleInput()
{
  cout << "In input_thread" << endl;

  char c;
  while((c = (char)cin.get()))
  {
    cout << "pushed: " << c << endl;
    realBuffer.push_back(c);
    input_dirty = true;
  }
}

void printVec(vector<char>& v) 
{
  cout << "Buf: ";
  for(char c : v)
  {
    cout << c;
  }
  cout << endl;
}

void printInput()
{
  while(!exit_prg)
  {
    if(input_dirty)
    {
      printVec(realBuffer);
      input_dirty = false;
    }
  }
}

int main(int argc, char** argv) {
  cout << "Init input" << endl;
  thread input_thread (handleInput);
  thread print_thread (printInput);

  input_thread.join();
  print_thread.join();
  return 0;
}
