#include <stdlib.h>
#include <iostream>
#include "Flags.h"

// Since Flags class is static,
// these need to be initialized after
// their static definition
std::map<std::string, Flags::Types> Flags::Registry;
std::map<std::string, Flags::IntFlag> Flags::IntFlags;
std::map<std::string, Flags::StringFlag> Flags::StringFlags;
std::map<std::string, Flags::BoolFlag> Flags::BoolFlags;

std::map<std::string, Flags::Types>::iterator Flags::RegistryIter;
std::map<std::string, Flags::IntFlag>::iterator Flags::IntFlagsIter;
std::map<std::string, Flags::StringFlag>::iterator Flags::StringFlagsIter;
std::map<std::string, Flags::BoolFlag>::iterator Flags::BoolFlagsIter;

void Flags::Int(int * value_ptr, int def_value, std::string name, std::string desc)
{
  *value_ptr = def_value;
  IntFlags[name] = {
    value_ptr,
    {
      name,
      desc
    }
  };
  Registry[name] = int_t;
}

void Flags::String(std::string * value_ptr, std::string def_value, std::string name, std::string desc)
{
  *value_ptr = def_value;
  StringFlags[name] = {
    value_ptr,
    {
      name,
      desc
    }
  };
  Registry[name] = string_t;
}

void Flags::Bool(bool * value_ptr, bool def_value, std::string name, std::string desc)
{
  *value_ptr = def_value;
  BoolFlags[name] = {
    value_ptr,
    {
      name,
      desc
    }
  };
  Registry[name] = bool_t;
}

void Flags::printHelp()
{
  for(RegistryIter = Registry.begin(); RegistryIter != Registry.end(); RegistryIter++)
  {
    std::string flag = RegistryIter->first;
    Types t = RegistryIter->second;
    switch(t) {
      case int_t:
        std::cout << "-" << flag << "\t|\t" << IntFlags[flag].meta.desc
          << "\t Default value (type Int): "
          << *IntFlags[flag].value
          << "\n";
        break;
      case string_t:
        std::cout << "-" << flag << "\t|\t" << StringFlags[flag].meta.desc
          << "\t Default value (type String): "
          << *StringFlags[flag].value
          << "\n";
        break;
      case bool_t:
        std::cout << "-" << flag << "\t|\t" << BoolFlags[flag].meta.desc
          << "\t Default value: (type Bool): "
          << *BoolFlags[flag].value
          << "\n";
        break;
      default:
        break;
    }
  }
}

void Flags::Parse(int argv, char ** argc)
{
  std::cout << "Argv: " << argv << std::endl;
  for(int i = 1; i < argv; i++)
  {
    std::cout << "Argc: " << argc[i] << std::endl;
    if(*argc[i] == '-')
    {
      std::string flag = argc[i] + 1;

      if (flag == "help") {
        printHelp();
        return;
      }

      switch(Registry[flag])
      {
        case int_t:
          IntFlagsIter = IntFlags.find(flag);
          if (IntFlagsIter == IntFlags.end()) break;
          *IntFlags[flag].value = atoi(argc[++i]);
          break;
        case string_t:
          StringFlagsIter = StringFlags.find(flag);
          if (StringFlagsIter == StringFlags.end()) break;
          *StringFlags[flag].value = (std::string)argc[++i];
          break;
        case bool_t:
          BoolFlagsIter = BoolFlags.find(flag);
          if (BoolFlagsIter == BoolFlags.end()) break;
          *BoolFlags[flag].value = !(*BoolFlags[flag].value);
          break;
        default:
          break;
      }
    }
  }
}

