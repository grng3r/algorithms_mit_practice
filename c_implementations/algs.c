//TODO Radix sort
#include <stdio.h>
void print_array(int arr[], int n){
	int i;
	for(i=0; i < n; i++)
		printf("%d,", arr[i]);
	printf("\n");
}


int main(){
	//int arr[] = {};
	//int arr[] = {1};
	//int arr [] = {1, 1};
	//int arr[] = {1, 2};
	//int arr[] = {2,1};
	int arr [] = {15, 31, 2, 54, 7, 8, 9, 23, 1};
	int len = sizeof(arr)/sizeof(arr[0]);
	introsort(arr, len);
	print_array(arr, len);
	//quicksort_desc(arr, len);
	//print_array(arr, len);
}


