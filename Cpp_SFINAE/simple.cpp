#include <iostream>
#include <type_traits>
#include <unordered_map>
#include <vector>


template<typename Ty_Counter>
class Counter
{
    Ty_Counter m_counter;
public:
    template<typename Ty>
    typename std::enable_if<std::is_same<Ty_Counter, std::unordered_map<Ty, int>>::value, int>::type
    incr(const Ty & key) {
        return ++m_counter[key];
    }
    
    template<typename Ty>
    typename std::enable_if<std::is_same<Ty_Counter, std::vector<Ty>>::value, int>::type
    incr(const Ty & idx) {
        if (m_counter.size() <= idx)
        {
            m_counter.resize(idx+1);
        }
        
        return ++m_counter[idx];
    }
    
    int incr() {
        return ++m_counter;
    }
};

int main(int argc, const char * argv[]) {
    Counter<int> intcounter;
    std::cout << intcounter.incr() << intcounter.incr() << intcounter.incr() << std::endl;
    
    Counter<std::vector<int>> veccounter;
    std::cout << veccounter.incr(0) << veccounter.incr(0) << veccounter.incr(1) << std::endl;
    
    Counter<std::unordered_map<int, int>> mapcounter;
    std::cout << mapcounter.incr(1) << mapcounter.incr(2) << mapcounter.incr(3) << mapcounter.incr(1) << std::endl;
    
    return 0;
}
