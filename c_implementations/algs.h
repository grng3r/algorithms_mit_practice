//tested for similar inputs as rust
//small nu of inputs(quadratic alg)
void insertion_sort_asc(long arr[], long len){
	int i, j; 
	long tmp;
	for(i = 1; i < len; i++){
		tmp = arr[i];
		j = i -1;
		while(j >= 0 && arr[j] > tmp){
			arr[j+1] = arr[j];
			j = j - 1;
		}
		arr[j+1] = tmp;
	}
}

//tested similar inputs rust
void insertion_sort_desc(long arr[], long len){
	int i, j; 
	long tmp;
	for(i = 1; i < len; i++){
		tmp = arr[i];
		j = i -1;
		while(j >= 0 && arr[j] < tmp){
			arr[j+1] = arr[j];
			j = j - 1;
		}
		arr[j+1] = tmp;
	}
}

void insertion_sort_asc(long arr[], long len){
	int i, j; 
	long tmp;
	for(i = 1; i < len; i++){
		tmp = arr[i];
		j = i -1;
		while(j >= 0 && arr[j] > tmp){
			arr[j+1] = arr[j];
			j = j - 1;
		}
		arr[j+1] = tmp;
	}
	//return arr;
}

void insertion_sort_desc(long arr[], long len){
	int i, j; 
	long tmp;
	for(i = 1; i < len; i++){
		tmp = arr[i];
		j = i -1;
		while(j >= 0 && arr[j] < tmp){
			arr[j+1] = arr[j];
			j = j - 1;
		}
		arr[j+1] = tmp;
	}
}


//merge sort help from hackr.io, www.cprogramming.com tutorial  and stackoverflow merge sort inplace question
//Built a iterrative merge sort rather than a recursive because the recursive one written in a couple of ways caused a segfault
#include <stdlib.h>
void merge_asc(int a[], int l,int m, int r){
	int i, j, k;
	int n1 = m - l + 1;
	int n2 = r-m;

	int *a1 = (int *)malloc(n1 * sizeof(int));
	int *a2 = (int *)malloc(n2 * sizeof(int));
	
	for(i = 0; i < n1; i++){
		a1[i] = a[l+i];
	}
	for(j = 0; j < n2; j++){
		a2[j] = a[m+1+j];
	}
	i = 0;
	j = 0;
	k = l;
	while(i < n1 && j < n2){
		if(a1[i] <= a2[j]){
			a[k] = a1[i];
			i++;
		} else{
			a[k] = a2[j];
			j++;
		}
		k++;
	}
	while(i < n1){
		a[k] = a1[i];
		i++;
		k++;
	}
	while(j < n2){
		a[k] = a2[j];
		j++;
		k++;
	}
	free(a1);
	free(a2);
}

int min(int x, int y){
	return(x<y) ? x:y;
}

void merge_sort_asc(int a[], int n){
	if (n < 2){
		return;
	} else if(n == 2){
		if(a[0] > a[1]){
			double tmp = a[0];
			a[0] = a[1];
			a[1] = tmp;
		} else{
			return;
		}
	}else{
			int curr_size, l;
			for(curr_size=1; curr_size <= n-1;curr_size=2*curr_size){
				for(l = 0; l < n-1; l += 2*curr_size){
					int m = min(l + curr_size-1, n-1);
					int r = min(l + 2*curr_size-1, n-1);
					merge_asc(a, l, m, r);

				}
			}
		}
}

void merge_desc(int a[], int l,int m, int r){
	int i, j, k;
	int n1 = m - l + 1;
	int n2 = r-m;

	int *a1 = (int *)malloc(n1 * sizeof(int));
	int *a2 = (int *)malloc(n2 * sizeof(int));
	
	for(i = 0; i < n1; i++){
		a1[i] = a[l+i];
	}
	for(j = 0; j < n2; j++){
		a2[j] = a[m+1+j];
	}
	i = 0;
	j = 0;
	k = l;
	while(i < n1 && j < n2){
		if(a1[i] >= a2[j]){
			a[k] = a1[i];
			i++;
		} else{
			a[k] = a2[j];
			j++;
		}
		k++;
	}
	while(i < n1){
		a[k] = a1[i];
		i++;
		k++;
	}
	while(j < n2){
		a[k] = a2[j];
		j++;
		k++;
	}
	free(a1);
	free(a2);

}

void merge_sort_desc(int a[], int n){
	if (n < 2){
		return;
	} else if(n == 2){
		if(a[0] < a[1]){
			double tmp = a[0];
			a[0] = a[1];
			a[1] = tmp;
		} else{
			return;
		}
	}else{
			int curr_size, l;
			for(curr_size=1; curr_size <= n-1;curr_size=2*curr_size){
				for(l = 0; l < n-1; l += 2*curr_size){
					int m = min(l + curr_size-1, n-1);
					int r = min(l + 2*curr_size-1, n-1);
					merge_desc(a, l, m, r);

				}
			}
		}
}



/*Inspired by Robert Sedgewick -> my optional extra so no descending order, although conditions for the while loop in the partition part of the code should be changed, and then it should work
 in reverse(lazy)*/ 
void qsort_asc(int *a, int start, int end){
	double tmp;
	if(end <= 1){
		return;
	} else if ((end == 2) && ((a[start] == a[end-1]) || (a[start] < a[end-1]))){
		return;
	}else if ((end == 2) && (a[start] > a[end-1])){
		tmp = a[0];
		a[0] = a[1];
		a[1] = tmp;	
	}else{
		int i, j, t, v;
		if(end > start){
			v = a[end];
			i = start-1;
			j = end;

			for(;;){//same as while(true)
				while(a[++i] < v);
				while(a[--j] > v);

				if(i>=j)
					break;
				t = a[i];
				a[i] = a[j];
				a[j] = t;
			}
			t = a[i];
			a[i] = a[end];
			a[end] = t;

			qsort_asc(a, start, i-1);
			qsort_asc(a, i+1, end);
		}
		return;
	}
}


//Max-Heap implementation
void heapify_asc(int a[], const int len, int root){
	int child = root;
	int l = 2 * root + 1;
	int r = l + 1;

	if(l < len && a[l] > a[child])
		child = l;
	if(r < len && a[r] > a[child])
		child = r;
	if(child != root){
		int t = a[root];
		a[root] = a[child];
		a[child] = t;

		heapify_asc(a, len, child);
	}

}

void heapsort_asc(int *a, const int len){
	if(len <= 1){
		return;
	} else if(len == 2 && a[0] > a[1]){
		int t = a[0];
		a[0] = a[1];
		a[1] = t;
	} else{
		int i;

		for(i = len/2-1; i >= 0; i--){
			heapify_asc(a, len, i);
		}

		for(i = len - 1; i >= 0; i--){
			int t = a[0];
			a[0] = a[i];
			a[i] = t;
			heapify_asc(a, i, 0);
		}
	}
}


