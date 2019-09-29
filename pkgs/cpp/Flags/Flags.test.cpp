#include "Flags.h"
#include <gtest/gtest.h>

int t_one;
int t_two;
bool t_three;
bool t_four;
std::string t_five;
std::string t_six;

TEST(FlagsParse, AllTypesWorking)
{

  ASSERT_EQ(t_one, 8080) << "Default Int Preserved";
  ASSERT_EQ(t_two, 1700) << "Default Int Overridden";
  ASSERT_FALSE(t_three) << "Default Bool preserved";
  ASSERT_TRUE(t_four) << "Default Bool Overridden";
  ASSERT_EQ(t_five, "default_str") << "Default String preserved";
  ASSERT_EQ(t_six, "https://changed.url") << "Default String Overridden";
}

int main(int argc, char ** argv) {
  ::testing::InitGoogleTest(&argc, argv);

  Flags::Int(&t_one, 8080, "t_one", "some_num");
  Flags::Int(&t_two, 8080, "t_two", "some_num");
  Flags::Bool(&t_three, false, "t_three", "some bool");
  Flags::Bool(&t_four, false, "t_four", "some other bool");
  Flags::String(&t_five, "default_str", "t_five", "some url");
  Flags::String(&t_six, "https://some.url", "t_six", "some string");

  Flags::Parse(argc, argv);

  return RUN_ALL_TESTS();
}
