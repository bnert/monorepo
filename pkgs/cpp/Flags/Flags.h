#include <string>
#include <map>

class Flags
{
  // Types
  struct FlagMeta
  {
    std::string name;
    std::string desc;
  };

  struct IntFlag
  {
    int * value;
    FlagMeta meta;
  };

  struct BoolFlag
  {
    bool * value;
    FlagMeta meta;
  };

  struct StringFlag
  {
    std::string * value;
    FlagMeta meta;
  };
  enum Types { int_t, string_t, bool_t };

  // Arg maps
  static std::map<std::string, Flags::Types> Registry;
  static std::map<std::string, Flags::IntFlag> IntFlags;
  static std::map<std::string, Flags::StringFlag> StringFlags;
  static std::map<std::string, Flags::BoolFlag> BoolFlags;

  // Arg Map iterators
  static std::map<std::string, Flags::Types>::iterator RegistryIter;
  static std::map<std::string, Flags::IntFlag>::iterator IntFlagsIter;
  static std::map<std::string, Flags::StringFlag>::iterator StringFlagsIter;
  static std::map<std::string, Flags::BoolFlag>::iterator BoolFlagsIter;

public:
  static void Int(int * value_ptr, int def_value, std::string name, std::string desc);
  static void String(std::string * value_ptr, std::string def_value, std::string name, std::string desc);
  static void Bool(bool * value_ptr, bool def_value, std::string name, std::string desc);
  static void printHelp();
  static void Parse(int argv, char ** argc);
};
