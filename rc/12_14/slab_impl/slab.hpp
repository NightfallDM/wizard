#ifndef _SLAB_HPP
#define _SLAB_HPP
#include <vector>

using std::vector;

namespace slab {
	template<class T>
	class Slab {
		private:
			vector<T> entries;
			unsigned long next;
			unsigned long _len;
			vector<unsigned long> free_vec;
			unsigned long free_cnt;
		public:
			Slab(): entries(vector<T>()), next(0), _len(0), free_vec(vector<unsigned long>()), free_cnt(0){}
			explicit Slab(unsigned long cap): entries(vector<T>(cap)), next(0), _len(0), free_vec(vector<unsigned long>(16)), free_cnt(0) {}
			
			unsigned long capacity() {
				return entries.size();
			}
			
			unsigned long len() {
				return _len;
			}

			unsigned long insert(T val) {
				if (free_cnt) {
					unsigned long ins_idx = free_vec.back();
					free_vec.pop_back();
					free_cnt--;
					entries[ins_idx] = val;
					_len++;
					return ins_idx;
				}else {
					entries[next] = val;
					next++;
					_len++;
					return next - 1;
				}
			}

			int remove(unsigned long key) {
				int free_idx;
				if (key >= next) {
					return -1;
				}
				
				for (free_idx = 0; free_idx < free_cnt; free_idx++) {
					if (free_vec[free_idx] == key) {
						return -1;
					}
				}
				free_vec.push_back(key);
				free_cnt++;
				return 0;
			}
	};
}
#endif /* end of _SLAB_HPP */
