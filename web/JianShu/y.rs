//rust--array��slice������

// ����array��slice�ı�����ʽ����[], ��������ȴ��Ȼ��ͬ.
// array ��rust���ǹ̶���С��, �������֮��Ͳ���������ɾ��array�е�Ԫ��.
// ��slice��δ֪��С��, ���һ�������slice������Ҫ���&��ʹ��.

// ������ʽ:
// array: [T; N]
// slice: [T]

// slice�ļ��ֱ�����ʽ:
// &[T]:        'share slice', often just called 'slice'.
// &mut [T]:    'mutable slice'
// Box<[T]>:    'boxed slice'


fn array_example() {
    let s: [i32; 4] = [1, 2, 3, 4];
    println!("{:?}", s);
}


fn slice_example() {
    let s: [i32; 4] = [1, 2, 3, 4];
    let y: &[i32] = &s[0..2];
    println!("{:?}", y);
}


fn main() {
    array_example();
    slice_example();
}