#include "pch.h"
#include "CppUnitTest.h"
#include "../coroutine-study/generator.hpp"

using namespace Microsoft::VisualStudio::CppUnitTestFramework;

namespace coroutinestudytest
{
	TEST_CLASS(generatortest)
	{
	public:
		
		TEST_METHOD(generator)
		{
			auto gen = generate_numbers(0, 10);
			auto expected = 0;
			while (auto value = gen.next()) {
				Assert::AreEqual(value.value(), expected++);
			}
		}
	};
}
